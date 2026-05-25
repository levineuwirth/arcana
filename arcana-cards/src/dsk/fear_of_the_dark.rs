//! Fear of the Dark — `{4}{B}` 5/5 Enchantment Creature — Nightmare.
//! "Whenever this creature attacks, if defending player controls no Glimmer creatures, it gains menace and deathtouch until end of turn. (A creature with menace can't be blocked except by two or more creatures.)"

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of the Dark");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::CREATURE | TypeLine::ENCHANTMENT),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: fear_of_the_dark_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn fear_of_the_dark_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump { target: trig.source, power: 0, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch] }]
}
