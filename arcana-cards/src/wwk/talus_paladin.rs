//! Talus Paladin — `{3}{W}` 2/3 Human Knight Ally.
//! "Whenever this creature or another Ally you control enters, you
//! may have Allies you control gain lifelink until end of turn, and
//! you may put a +1/+1 counter on this creature."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talus Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Ally")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_ally_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_ally_enters(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let ally_ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Ally").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects: Vec<Effect> = ally_ids
        .into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        })
        .collect();
    effects.push(Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    });
    effects
}
