//! Ill-Tempered Loner // Howlpack Avenger — `{2}{R}{R}` red Creature — Human Werewolf / Werewolf.
//!
//! Front face (Ill-Tempered Loner): 3/3
//!   Whenever this creature is dealt damage, it deals that much damage to any target.
//!   {1}{R}: This creature gets +2/+0 until end of turn.
//!   Daybound (not modeled; see GAP below).
//!
//! Back face (Howlpack Avenger): (no P/T spec provided; using 5/5 as per oracle)
//!   Whenever a permanent you control is dealt damage, this creature deals that
//!   much damage to any target.
//!   {1}{R}: This creature gets +2/+0 until end of turn.
//!   Nightbound (not modeled; see GAP below).
//!
//! GAP: Daybound/Nightbound keyword — day/night cycle not modeled in engine.
//! GAP: "Whenever this creature is dealt damage" — DamageReceived trigger with
//!      amount forwarding to any target is not in the current TriggerCondition catalog;
//!      the trigger body is GAP'd.
//! GAP: "Whenever a permanent you control is dealt damage" (back face) — similarly
//!      not expressible. Back-face-only triggered ability not auto-installed.
//! GAP: Transform conditions (day→night / night→day) not modeled; transform
//!      trigger not wired (no suitable TriggerCondition for day/night transitions).
//! The activated +2/+0 ability is modeled on the front face (works for both faces
//! via CardDefinition shared ability).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ill-Tempered Loner");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Daybound keyword not in engine keyword surface
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Howlpack Avenger");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            // GAP: Nightbound keyword not in engine keyword surface
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {1}{R}: +2/+0 until end of turn (shared by both faces via CardDefinition)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +2/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: pump_two_zero,
            }),
        // GAP: "Whenever this creature is dealt damage, it deals that much damage
        //      to any target" — DamageReceived trigger with forwarded amount not
        //      in TriggerCondition catalog.
        // GAP: Day/night transform triggers not modeled (no engine support for
        //      day/night cycle).
        // GAP: Back-face-only "whenever a permanent you control is dealt damage"
        //      not auto-installed on transform.
    )
}

fn pump_two_zero(
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
