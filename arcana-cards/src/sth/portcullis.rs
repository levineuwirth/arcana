//! Portcullis — `{4}` artifact (Stronghold, 1998).
//! "Whenever a creature enters, if there are two or more other creatures
//! on the battlefield, exile that creature. Return that card to the
//! battlefield under its owner's control when this artifact leaves the
//! battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Portcullis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: Some(if_two_other_creatures),
                effect: exile_entering_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn if_two_other_creatures(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "two or more OTHER creatures on the battlefield" — the entering
    // creature is already on the battlefield when the condition is
    // checked, so require at least three creatures total.
    script::count_matching(s, &ObjectFilter::creature(), you) >= 3
}

fn exile_entering_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return that card to the battlefield under its owner's control
    // when this artifact leaves the battlefield" — the linked
    // return-on-leave is not expressible (DelayedWhen has no SourceLeaves
    // tied to another object's exile).
    let Some(target) = trig.entering_object() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target }]
}
