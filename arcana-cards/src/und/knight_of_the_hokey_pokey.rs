//! Knight of the Hokey Pokey — `{W}{W}` 2/2 Knight with First strike.
//! "{1}{W}, Do the Hokey Pokey: Prevent all damage a source of your
//! choice would deal to this creature this turn."
//!
//! First strike is a base keyword. The activated ability prevents
//! damage to this creature this turn. The "Do the Hokey Pokey"
//! additional cost is Un-set flavor with no mechanical effect, so the
//! cost is modeled as mana only. "A source of your choice" is
//! simplified to all sources (the engine's PreventDamage with no
//! source restriction) — a fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Hokey Pokey");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}, Do the Hokey Pokey: Prevent all damage a source of your choice would deal to this creature this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_damage_to_self,
            }),
    )
}

fn prevent_damage_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
