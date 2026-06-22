//! Deathbringer Regent — `{5}{B}{B}` 5/6 Dragon with Flying.
//!
//! Oracle:
//! * Flying
//! * When this creature enters, if you cast it from your hand and there
//!   are five or more other creatures on the battlefield, destroy all
//!   other creatures. (The intervening-if gates on "five or more other
//!   creatures" — wired as total battlefield creatures >= 6, i.e. five
//!   besides this one. GAP: the "if you cast it from your hand" half has
//!   no accessor and is omitted from the gate — a documented fidelity
//!   partial that may fire on non-hand casts.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathbringer Regent");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(if_five_or_more_other_creatures),
            effect: destroy_other_creatures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_five_or_more_other_creatures(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // Five or more OTHER creatures = at least six creatures on the
    // battlefield including this one.
    // GAP: "if you cast it from your hand" half of the gate is omitted —
    // no cast-source accessor is available.
    script::count_matching(s, &ObjectFilter::creature(), you) >= 6
}

fn destroy_other_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets: Vec<ObjectId> = script::ids_matching(state, &ObjectFilter::creature(), trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}
