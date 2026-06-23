//! Metamorphosis Fanatic — `{4}{B}{B}` 4/4 black Human Cleric.
//!
//! Oracle:
//! * Lifelink.
//! * When this creature enters, return up to one target creature card
//!   from your graveyard to the battlefield with a lifelink counter on
//!   it.
//! * Miracle {1}{B} — alternative cast cost. GAP: the miracle
//!   cast-window mechanic is not expressible with the MultiAbilityCreature
//!   primitives (it is neither a triggered nor an activated ability on
//!   the battlefield); only the keyword line / triggers are wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Metamorphosis Fanatic");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the named counter so the resolver's lookup succeeds.
    let _lifelink = reg.interner_mut().intern("lifelink");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_reanimate_with_lifelink_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn etb_reanimate_with_lifelink_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "lifelink counter" — no dedicated CounterKind variant; record it as a
    // named counter so the catalog is faithful (the keyword-granting rider
    // is engine debt). The dedicated re-id-aware variant places the counter
    // on the freshly re-entered object.
    let lifelink = reg.interner().lookup("lifelink").unwrap_or_default();
    vec![Effect::ReturnFromGraveyardWithCounters {
        target: *id,
        kind: CounterKind::Named(lifelink),
        count: 1,
    }]
}
