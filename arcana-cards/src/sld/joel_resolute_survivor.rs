//! Joel, Resolute Survivor — `{3}{B}{G}` 4/4 Legendary Creature — Human Survivor.
//! Menace.
//! Whenever a creature token dies, put a +1/+1 counter on Joel and draw a card.
//! This ability triggers only once each turn.
//! Partner—Survivors.
//!
//! GAP: "Partner—Survivors" — the partner commander mechanic is not an
//! expressible KeywordAbility variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joel, Resolute Survivor");
    let human = reg.interner_mut().intern("Human");
    let survivor = reg.interner_mut().intern("Survivor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(survivor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().tokens_only(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: grow_and_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn grow_and_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ])]
}
