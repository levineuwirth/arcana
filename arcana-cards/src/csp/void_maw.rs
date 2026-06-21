//! Void Maw — `{4}{B}{B}` 4/5 Horror with Trample.
//! "If another creature would die, exile it instead.
//!  Put a card exiled with this creature into its owner's graveyard:
//!  This creature gets +2/+2 until end of turn."
//!
//! Trample is a base keyword. The two non-keyword clauses both reference
//! Void Maw's bespoke exile-replacement + linked-exile zone, which the
//! demonstrated API does not model:
//! * GAP: "If another creature would die, exile it instead." — this is a
//!   continuous death-replacement effect; there is no death-replacement
//!   Effect/ContinuousEffect primitive in the demonstrated surface.
//! * The activated ability's COST is "put a card exiled with this creature
//!   into its owner's graveyard" — there is no ActivationCost field for
//!   returning a linked-exiled card, and the linked-exile zone itself is
//!   GAP'd. The +2/+2 payload IS expressible, but the cost is not, so the
//!   ability is recorded with a best-effort no-cost approximation GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Void Maw");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "If another creature would die, exile it instead." — death-
    // replacement continuous effect, no primitive in the demonstrated API.
    // GAP: the activated ability's cost ("put a card exiled with this
    // creature into its owner's graveyard") relies on Void Maw's linked
    // exile zone, which is unmodeled; no ActivationCost field expresses it,
    // so the otherwise-trivial +2/+2 payload is omitted rather than wired
    // with a free (cost-less) approximation.
    reg.register(CardDefinition::new(name, chars))
}
