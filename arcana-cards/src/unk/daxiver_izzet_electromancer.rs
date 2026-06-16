//! Daxiver, Izzet Electromancer — `{1}{U}{R}` 2/4 Legendary Goblin Wizard.
//! "{T}: Add one mana of any color. Spend this mana only to cast instant and
//!  sorcery spells."
//!
//! The "Rulebreaker" deck-building static (Commander color-identity exemption)
//! is a deck-construction rule, not a battlefield ability — GAP'd.
//! The mana ability is wired; "any color" has no engine variant so a
//! representative color is emitted, and the spend restriction is not
//! expressible as a mana rider — both noted as GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daxiver, Izzet Electromancer");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);

    // GAP: "Rulebreaker" — Commander deck-building color-identity exemption is
    // a deck-construction rule, not a battlefield ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add one mana of any color. Spend this mana only to cast instant and sorcery spells.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_any_color,
        }),
    )
}

fn add_any_color(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "any color" choice — AddMana requires a specific color; emitting
    // Blue as an approximation. GAP: "Spend this mana only to cast instant and
    // sorcery spells" — no spend-restriction rider on AddMana.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
