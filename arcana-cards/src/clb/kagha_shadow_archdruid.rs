//! Kagha, Shadow Archdruid — `{2}{B}{G}` 1/4 Legendary Elf Druid.
//! "Whenever Kagha attacks, it gains deathtouch until end of turn. Mill two
//!  cards."
//! "Once during each of your turns, you may play a land or cast a permanent
//!  spell from among cards in your graveyard that were put there from your
//!  library this turn." (GAP)
//!
//! GAP: the graveyard play-permission static has no expressible effect.
//! "Mill" is a Scryfall ability word, not a usable `KeywordAbility` —
//! `keywords: vec![]`.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kagha, Shadow Archdruid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Once during each of your turns, you may play a land or cast a
    //      permanent spell from your graveyard…" — no expressible effect.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: deathtouch_and_mill,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn deathtouch_and_mill(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
        Effect::Mill {
            player: trig.controller,
            count: 2,
        },
    ]
}
