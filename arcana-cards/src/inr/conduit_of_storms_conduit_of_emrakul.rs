//! Conduit of Storms // Conduit of Emrakul — `{2}{R}` Werewolf Horror 2/3.
//! Front face: Whenever this creature attacks, add {R} at the beginning
//! of your next main phase this turn. {3}{R}{R}: Transform this creature.
//! Back face (Conduit of Emrakul): Whenever this creature attacks, add
//! {C}{C} at the beginning of your next main phase this turn.
//!
//! GAP: "at the beginning of your next main phase this turn" is a delayed
//! triggered mana ability; the engine does not model delayed mana triggers.
//! Mana is added immediately on the attack trigger as the closest expressible
//! approximation.
//! GAP: back-face-only triggered ability (add {C}{C} on attack) is not
//! auto-installed on transform — back-face-only triggered ability not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conduit of Storms");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf_sub);
    subtypes.0.insert(horror_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Conduit of Emrakul");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi_sub);
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever this creature attacks, add {R} (approximation
            // of "at the beginning of your next main phase this turn").
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks_add_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {3}{R}{R}: Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{R}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: transform_self,
            }),
    )
}

fn on_attacks_add_red(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at the beginning of your next main phase this turn" not modeled;
    // adding mana immediately as the closest approximation.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
