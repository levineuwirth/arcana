//! Voice of Resurgence — `{G}{W}` 2/2 green/white Creature — Elemental.
//! "Whenever an opponent casts a spell during your turn and when this
//! creature dies, create a green and white Elemental creature token with
//! 'This token's power and toughness are each equal to the number of
//! creatures you control.'"
//!
//! The token's characteristic-defining ability is a printed static of the
//! token, present as it enters (A1.5br). It counts creatures by printed
//! type: a permanent a continuous effect has made a creature, or made
//! not one, is miscounted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voice of Resurgence");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Custom(opponent_spell_on_your_turn),
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_elemental_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg
        .interner()
        .lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    // "This token's power and toughness are each equal to the number of
    // creatures you control" (CR 604.3). Without the ability the token is
    // 0/0, as a `*`/`*` creature that has lost its abilities is.
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    }
    .with_static(vec![ContinuousEffect::self_pt_from_match(
        NULL_OBJECT_ID,
        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        Duration::WhileSourceOnBattlefield,
    )]);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn opponent_spell_on_your_turn(
    event: &arcana_core::events::GameEvent,
    state: &GameState,
    source: arcana_core::objects::ObjectId,
) -> bool {
    let Some(obj) = state.objects.get(source) else {
        return false;
    };
    state.active_player() == obj.controller
        && TriggerCondition::SpellCast {
            filter: None,
            caster: ControllerConstraint::Opponent,
        }
        .matches(event, source, obj.controller, state)
}
