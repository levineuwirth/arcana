//! Reveille Squad — `{2}{W}{W}` 3/3 Human Rebel. "Whenever one or more
//! creatures attack you, if this creature is untapped, you may untap all
//! creatures you control."
//!
//! "creatures attack you" modeled via `CreatureAttacks` with an
//! `attacking_you_only()` filter (defending player == this card's
//! controller). "if this creature is untapped" modeled via
//! `intervening_if` reading the source's tap state. Effect "untap all
//! creatures you control" uses ForEach + script::ids_matching.
//! GAP: "one or more creatures attack you" should fire once per attack
//! event; EachTime fires once per attacking creature (untap is
//! idempotent, and "you may" is auto-yes).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reveille Squad");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "creatures attack you" — attacking_you_only() restricts to
                // attackers whose defending player is this card's controller.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().attacking_you_only(),
                },
                // "if this creature is untapped" via the source's tap state.
                intervening_if: Some(iif_source_untapped),
                effect: untap_all_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_source_untapped(state: &GameState, source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    state.objects.get(source).is_some_and(|o| !o.is_tapped())
}

fn untap_all_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
    }]
}
