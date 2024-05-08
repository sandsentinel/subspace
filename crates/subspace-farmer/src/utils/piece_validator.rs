use crate::node_client::NodeClient;
use async_trait::async_trait;
use subspace_archiving::archiver::is_piece_valid;
use subspace_core_primitives::crypto::kzg::Kzg;
use subspace_core_primitives::{Piece, PieceIndex};
use subspace_networking::libp2p::PeerId;
use subspace_networking::utils::piece_provider::PieceValidator;
use subspace_networking::Node;
use tracing::{error, warn};

#[derive(Clone)]
pub struct SegmentCommitmentPieceValidator<NC> {
    dsn_node: Node,
    node_client: NC,
    kzg: Kzg,
}

impl<NC> SegmentCommitmentPieceValidator<NC> {
    pub fn new(dsn_node: Node, node_client: NC, kzg: Kzg) -> Self {
        Self {
            dsn_node,
            node_client,
            kzg,
        }
    }
}

#[async_trait]
impl<NC> PieceValidator for SegmentCommitmentPieceValidator<NC>
where
    NC: NodeClient,
{
    async fn validate_piece(
        &self,
        source_peer_id: PeerId,
        piece_index: PieceIndex,
        piece: Piece,
    ) -> Option<Piece> {
        if source_peer_id == self.dsn_node.id() {
            return Some(piece);
        }

        let segment_index = piece_index.segment_index();

        let segment_headers = match self.node_client.segment_headers(vec![segment_index]).await {
            Ok(segment_headers) => segment_headers,
            Err(error) => {
                error!(
                    %piece_index,
                    ?error,
                    "Failed tor retrieve segment headers from node"
                );
                return None;
            }
        };

        let segment_commitment = match segment_headers.into_iter().next().flatten() {
            Some(segment_header) => segment_header.segment_commitment(),
            None => {
                error!(
                    %piece_index,
                    %segment_index,
                    "Segment commitment for segment index wasn't found on node"
                );
                return None;
            }
        };

        let is_valid_fut = tokio::task::spawn_blocking({
            let kzg = self.kzg.clone();

            move || {
                is_piece_valid(&kzg, &piece, &segment_commitment, piece_index.position())
                    .then_some(piece)
            }
        });

        match is_valid_fut.await.unwrap_or_default() {
            Some(piece) => Some(piece),
            None => {
                warn!(
                    %piece_index,
                    %source_peer_id,
                    "Received invalid piece from peer"
                );

                // We don't care about result here
                let _ = self.dsn_node.ban_peer(source_peer_id).await;
                None
            }
        }
    }
}
