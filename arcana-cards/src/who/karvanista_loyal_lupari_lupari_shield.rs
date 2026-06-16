//! Karvanista, Loyal Lupari // Lupari Shield — `{4}{G}` 5/5 green Legendary Alien Dog Soldier.
//! Vigilance, trample, haste.
//! "Whenever Karvanista attacks, put a +1/+1 counter on each Human you control."
//! Adventure face "Lupari Shield" (`{1}{G}` Sorcery):
//! "Humans you control gain indestructible until your next turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karvanista, Loyal Lupari");
    let alien = reg.interner_mut().intern("Alien");
    let dog = reg.interner_mut().intern("Dog");
    let soldier = reg.interner_mut().intern("Soldier");
    let _human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(dog);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Lupari Shield");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid adv cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Humans you control gain indestructible until your next turn.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: lupari_shield,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_humans,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn pump_humans(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Human")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

fn lupari_shield(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Human")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::UntilYourNextTurn(entry.controller),
        }),
    }]
}
