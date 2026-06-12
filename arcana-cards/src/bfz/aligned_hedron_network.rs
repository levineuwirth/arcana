//! Aligned Hedron Network — `{4}` artifact (Battle for Zendikar, 2015).
//! "When this artifact enters, exile all creatures with power 5 or
//! greater until this artifact leaves the battlefield." Board-wide
//! exile wired as one `Effect::ExileUntilSourceLeaves` per power >= 5
//! creature — the engine returns the linked batch when this artifact
//! leaves the battlefield (CR 610.3).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aligned Hedron Network");
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_big_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn exile_big_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // One ExileUntilSourceLeaves per match: the engine returns each linked
    // creature when this artifact leaves the battlefield.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_min_power(5),
        trig.controller,
    );
    ids.into_iter()
        .map(|id| Effect::ExileUntilSourceLeaves {
            source: trig.source,
            target: id,
        })
        .collect()
}
