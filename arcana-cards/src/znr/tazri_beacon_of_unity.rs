//! Tazri, Beacon of Unity — `{4}{W}` 4/6 Legendary Human Warrior.
//! "This spell costs {1} less to cast for each creature in your party.
//! {2/U}{2/B}{2/R}{2/G}: Look at the top six cards of your library. You may reveal
//! up to two Cleric, Rogue, Warrior, Wizard, and/or Ally cards from among them and
//! put them into your hand. Put the rest on the bottom of your library in a random
//! order."
//!
//! The party-based cost reduction is a static cost modifier with no representation
//! in the demonstrated API (GAP). The activated dig ability is modeled best-effort
//! with DigTopN, which is single-take ("up to two into hand" is a documented
//! fidelity gap — DigTopN takes at most one matching card).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tazri, Beacon of Unity");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "costs {1} less for each creature in your party" — no cost-reduction
        // static modifier in the demonstrated API.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2/U}{2/B}{2/R}{2/G}: Look at the top six cards of your library. You may reveal up to two Cleric, Rogue, Warrior, Wizard, and/or Ally cards from among them and put them into your hand. Put the rest on the bottom of your library in a random order.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2/U}{2/B}{2/R}{2/G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: dig_for_party,
        }),
    )
}

fn dig_for_party(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let mut subs = Vec::new();
    for s in ["Cleric", "Rogue", "Warrior", "Wizard", "Ally"] {
        if let Some(id) = reg.interner().lookup(s) {
            subs.push(id);
        }
    }
    let filter = ObjectFilter::new().with_subtypes_any(subs);
    // Fidelity gap: oracle takes UP TO TWO matching cards; DigTopN takes one.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 6,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
