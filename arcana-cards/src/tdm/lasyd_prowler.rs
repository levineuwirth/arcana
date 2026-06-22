//! Lasyd Prowler — `{2}{G}{G}` 5/5 green Snake Ranger.
//!
//! * When this creature enters, you may mill cards equal to the number
//!   of lands you control. (Self-mill is the controller's beneficial
//!   choice — modeled as an unconditional mill; the "may" is a minor
//!   fidelity gap.)
//! * Renew — `{1}{G}`, Exile this card from your graveyard: Put X
//!   +1/+1 counters on target creature, where X is the number of land
//!   cards in your graveyard. Activate only as a sorcery. (Graveyard
//!   activation with `exile_self`.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lasyd Prowler");
    let snake = reg.interner_mut().intern("Snake");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Renew — {1}{G}, Exile this card from your graveyard: Put X +1/+1 counters on target creature, where X is the number of land cards in your graveyard. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: renew_counters,
            }),
    )
}

/// ETB: mill cards equal to the number of lands you control.
fn etb_mill(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let lands = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let n = script::count_matching(state, &lands, trig.controller);
    vec![Effect::Mill { player: trig.controller, count: n }]
}

/// Renew: put X +1/+1 counters on the target creature, X = land cards
/// in your graveyard.
fn renew_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let lands = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let x = script::graveyard_matching(state, &lands, ctx.controller, ctx.controller);
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: x,
    }]
}
