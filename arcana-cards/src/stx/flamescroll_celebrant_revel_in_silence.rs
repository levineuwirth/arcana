//! Flamescroll Celebrant // Revel in Silence — MDFC.
//! Front: `{1}{R}` Creature — Human Shaman 2/1.
//!   Whenever an opponent activates an ability that isn't a mana ability,
//!   this creature deals 1 damage to that player.
//!   (GAP: no TriggerCondition for "opponent activates non-mana ability".)
//!   {1}{R}: This creature gets +2/+0 until end of turn.
//! Back: Revel in Silence — Instant.
//!   Your opponents can't cast spells or activate planeswalkers' loyalty
//!   abilities this turn. Exile Revel in Silence.
//!   (GAP: "opponents can't cast spells / activate loyalty abilities" — no
//!   Effect variant for blanket cast-lock or loyalty-lock.)
//!
//! # GAPs
//! - Front triggered ability "whenever an opponent activates a non-mana ability"
//!   has no TriggerCondition — not modeled.
//! - Back-face effect "opponents can't cast spells or activate loyalty abilities
//!   this turn" has no Effect variant — not modeled.
//! - GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flamescroll Celebrant");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Revel in Silence — Instant
    let back_name = reg.interner_mut().intern("Revel in Silence");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid back cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let back_ability = SpellAbilityDef {
        text: "Your opponents can't cast spells or activate planeswalkers' loyalty abilities this turn. Exile Revel in Silence.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: revel_resolve,
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: Some(back_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "whenever an opponent activates a non-mana ability" — no TriggerCondition
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +2/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: pump_self,
            })
            .with_mdfc_back(back_face)
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn revel_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "opponents can't cast spells or activate loyalty abilities this turn"
    // — no Effect variant for blanket cast-lock or loyalty-lock.
    // GAP: "Exile Revel in Silence" — self-exile on resolution; engine routes to
    // graveyard by default for instants.
    Vec::new()
}
