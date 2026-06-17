//! Sarkhan, Dragon Ascendant — `{1}{R}` 2/2 Legendary Human Druid.
//! When Sarkhan enters, you may behold a Dragon. If you do, create a
//! Treasure token.
//! Whenever a Dragon you control enters, put a +1/+1 counter on Sarkhan.
//! Until end of turn, Sarkhan becomes a Dragon in addition to its other
//! types and gains flying.
//!
//! "Behold" / "Treasure" are mechanic markers, not KeywordAbility
//! variants, so keywords is empty. The ETB "may behold a Dragon → create
//! a Treasure" effect is gated on the unmodelable behold choice, so its
//! effect is GAP'd. The Dragon-enters trigger is fully expressed.

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
    let name = reg.interner_mut().intern("Sarkhan, Dragon Ascendant");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let dragon_enters = script::subtype_filter(reg, "Dragon")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_behold,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: dragon_enters,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: dragon_entered,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_behold(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may behold a Dragon. If you do, create a Treasure token."
    //      The behold choice (reveal/choose a Dragon) is not modelable, so
    //      the conditional Treasure creation is GAP'd in full.
    Vec::new()
}

fn dragon_entered(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Sarkhan becomes a Dragon in addition to its other types" —
    //      adding a creature SUBTYPE for a duration has no expressible
    //      primitive (AddType handles card types only). The +1/+1 counter
    //      and the end-of-turn Flying grant are emitted.
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}
