use crate::{Block, Runtime, RuntimeCall};
use codec::{Compact, CompactLen, Encode};
use sp_std::iter::Peekable;
use sp_std::prelude::*;
use subspace_core_primitives::crypto;
use subspace_core_primitives::objects::{BlockObject, BlockObjectMapping};
use subspace_runtime_primitives::Hash;

const MAX_OBJECT_MAPPING_RECURSION_DEPTH: u16 = 5;

pub(crate) fn extract_utility_block_object_mapping<I: Iterator<Item = Hash>>(
    mut base_offset: u32,
    objects: &mut Vec<BlockObject>,
    call: &pallet_utility::Call<Runtime>,
    mut recursion_depth_left: u16,
    successful_calls: &mut Peekable<I>,
) {
    if recursion_depth_left == 0 {
        return;
    }

    recursion_depth_left -= 1;

    // Add enum variant to the base offset.
    base_offset += 1;

    match call {
        pallet_utility::Call::batch { calls }
        | pallet_utility::Call::batch_all { calls }
        | pallet_utility::Call::force_batch { calls } => {
            base_offset += Compact::compact_len(&(calls.len() as u32)) as u32;

            for call in calls {
                extract_call_block_object_mapping(
                    base_offset,
                    objects,
                    call,
                    recursion_depth_left,
                    successful_calls,
                );

                base_offset += call.encoded_size() as u32;
            }
        }
        pallet_utility::Call::as_derivative { index, call } => {
            base_offset += index.encoded_size() as u32;

            extract_call_block_object_mapping(
                base_offset,
                objects,
                call.as_ref(),
                recursion_depth_left,
                successful_calls,
            );
        }
        pallet_utility::Call::dispatch_as { as_origin, call } => {
            base_offset += as_origin.encoded_size() as u32;

            extract_call_block_object_mapping(
                base_offset,
                objects,
                call.as_ref(),
                recursion_depth_left,
                successful_calls,
            );
        }
        pallet_utility::Call::with_weight { call, .. } => {
            extract_call_block_object_mapping(
                base_offset,
                objects,
                call.as_ref(),
                recursion_depth_left,
                successful_calls,
            );
        }
        pallet_utility::Call::__Ignore(_, _) => {
            // Ignore.
        }
    }
}

pub(crate) fn extract_call_block_object_mapping<I: Iterator<Item = Hash>>(
    mut base_offset: u32,
    objects: &mut Vec<BlockObject>,
    call: &RuntimeCall,
    recursion_depth_left: u16,
    successful_calls: &mut Peekable<I>,
) {
    // Add enum variant to the base offset.
    base_offset += 1;

    match call {
        // Extract the actual object mappings.
        RuntimeCall::System(frame_system::Call::remark { remark }) => {
            objects.push(BlockObject::V0 {
                hash: crypto::blake3_hash(remark),
                offset: base_offset,
            });
        }
        // Recursively extract object mappings for the call.
        RuntimeCall::Utility(call) => extract_utility_block_object_mapping(
            base_offset,
            objects,
            call,
            recursion_depth_left,
            successful_calls,
        ),
        // Other calls don't contain object mappings.
        _ => {}
    }
}

pub(crate) fn extract_block_object_mapping(
    block: Block,
    successful_calls: Vec<Hash>,
) -> BlockObjectMapping {
    let mut block_object_mapping = BlockObjectMapping::default();
    let mut successful_calls = successful_calls.into_iter().peekable();
    let mut base_offset =
        block.header.encoded_size() + Compact::compact_len(&(block.extrinsics.len() as u32));
    for extrinsic in block.extrinsics {
        let signature_size = extrinsic
            .signature
            .as_ref()
            .map(|s| s.encoded_size())
            .unwrap_or_default();
        // Extrinsic starts with vector length and version byte, followed by optional signature and
        // `function` encoding.
        let base_extrinsic_offset = base_offset
            + Compact::compact_len(
                &((1 + signature_size + extrinsic.function.encoded_size()) as u32),
            )
            + 1
            + signature_size;

        extract_call_block_object_mapping(
            base_extrinsic_offset as u32,
            &mut block_object_mapping.objects,
            &extrinsic.function,
            MAX_OBJECT_MAPPING_RECURSION_DEPTH,
            &mut successful_calls,
        );

        base_offset += extrinsic.encoded_size();
    }

    block_object_mapping
}
