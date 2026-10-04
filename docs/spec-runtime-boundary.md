# Spec ↔ Runtime boundary

The inspected remote main of industry-world-model was `14f05caba9c157f75f18cb8ecb987423d35b850d`. It contains portable protocol v0.3, packs and an executable generic API. Its multi-world RFC recognizes current pack-scoped identities and proposes global identity for a later protocol version. Kernel does not claim that this migration has occurred.

Protocol owns contribution packets, review and portable validation. Kernel owns executable runtime demand/resolve/hydrate behavior. Kernel v0.1 envelopes are internal adapter DTOs, not replacements or byte-compatible copies of protocol contribution packets. `SemanticKey` is runtime normalized demand vocabulary; mapping to protocol predicates must be explicit and versioned.

No schemas are vendored. Reference the upstream protocol and negotiate a pinned supported version in each future adapter. Unsupported fields/status/time semantics must return error or unknown; never silently promote proposed claims. A future adapter needs protocol fixtures and compatibility tests before declaring conformance.

Pack-scoped IDs cannot be mechanically converted into global Reality IDs by stripping a pack prefix. Identity mapping needs an audited canonical mapping service with ambiguity handling. Governance remains outside this repository behind policy hooks.
