//! Slivdrazi Monstrosity — `{C}{W}{U}{B}{R}{G}` 8/8 Legendary Creature —
//! Sliver Eldrazi (all five colors).
//!
//! * "Eldrazi you control are Slivers in addition to their other types." —
//!   static type-grant over a board set. GAP (no static type-add ability).
//! * "Slivers you control have devoid and annihilator 1." — static
//!   keyword-grant. GAP.
//! * `{3}: Create a 1/1 colorless Eldrazi Sliver creature token. It has
//!   "Sacrifice this creature: Add {C}."` — the token is minted; its
//!   sacrifice-for-mana intrinsic ability is GAP'd (inner activated
//!   abilities on a TokenDefinition are not expressible here).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slivdrazi Monstrosity");
    let sliver = reg.interner_mut().intern("Sliver");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{C}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: arcana_core::types::SupertypeSet(arcana_core::types::SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    // GAP: "Eldrazi you control are Slivers in addition to their other types" (static type-add).
    // GAP: "Slivers you control have devoid and annihilator 1" (static keyword-grant).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: Create a 1/1 colorless Eldrazi Sliver creature token. It has \"Sacrifice this creature: Add {C}.\""
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_token,
        }),
    )
}

fn make_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let sliver = reg.interner().lookup("Sliver").unwrap_or_default();
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let name = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(sliver);
    // GAP: token's "Sacrifice this creature: Add {C}" intrinsic ability not expressible.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
