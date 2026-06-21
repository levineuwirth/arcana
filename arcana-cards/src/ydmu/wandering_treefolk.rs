//! Wandering Treefolk — `{1}{G}` 2/3 Creature — Treefolk with Vigilance.
//!
//! Oracle:
//! * Vigilance.
//! * "Domain — {7}{G}: Seek a creature card. This ability costs {1} less
//!   to activate for each basic land type among lands you control." — a
//!   mana activation at its printed {7}{G} cost. GAP: "seek" is not a
//!   documented effect (random library-search without revealing), so the
//!   effect body is empty. GAP: the Domain cost reduction ("costs {1}
//!   less for each basic land type among lands you control") is not an
//!   expressible dynamic cost modifier.
//!
//! Keywords Seek and Domain are not in the usable keyword surface.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wandering Treefolk");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: Domain cost reduction ("{1} less for each basic land type
            // among lands you control") not expressible; printed cost used.
            text: "Domain — {7}{G}: Seek a creature card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: seek_creature,
        }),
    )
}

fn seek_creature(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek" is not a documented effect (random library-search
    // without revealing); not expressible with the demonstrated API.
    Vec::new()
}
