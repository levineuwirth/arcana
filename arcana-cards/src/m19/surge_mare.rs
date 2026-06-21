//! Surge Mare — `{U}{U}` 0/5 Horse Fish.
//!
//! "This creature can't be blocked by green creatures.
//!  Whenever this creature deals damage to an opponent, you may draw a
//!  card. If you do, discard a card.
//!  {1}{U}: This creature gets +2/-2 until end of turn."
//!
//! The "can't be blocked by green creatures" static is a color-keyed block
//! restriction with no available primitive (Fear/Intimidate/Shadow are the
//! only color/type evasions), so it is GAP'd. The loot trigger models the
//! "may draw / if you do, discard" as draw 1 then discard 1 (the optional
//! "may" is a Phase-1 fidelity simplification — there is no optional-draw
//! gate primitive). The +2/-2 pump is a plain mana-cost activated ability.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge Mare");
    let horse = reg.interner_mut().intern("Horse");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "can't be blocked by green creatures" — no color-keyed
    // block-restriction primitive (only Fear/Intimidate/Shadow evasions).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: loot_on_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: This creature gets +2/-2 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn loot_on_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Restrict to an opponent of the controller (the trigger fires for
    // any damaged player; only act when it is an opponent).
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    if !script::opponents(state, trig.controller).contains(&p) {
        return Vec::new();
    }
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: -2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
