//! Albiorix, Goose Tyrant // Wild Goose Chase — `{1}{G}{U}` // `{G}{U}` green/blue Adventure creature.
//! Legendary Creature — Bird. Flying, Trample, Ward {1}.
//! Whenever you sacrifice a token, Albiorix perpetually gets +1/+1 (also in exile).
//! Adventure (Wild Goose Chase — Instant): Draw two cards, then discard two cards. Create a Food token.
//! GAP: "perpetually gets +1/+1" (Arena-only perpetual mechanic) — not in catalog.
//! GAP: "this ability also triggers if Albiorix is in exile" — trigger from exile not in engine.

use arcana_core::effects::{CommodityToken, DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Albiorix, Goose Tyrant");
    let adv_name = reg.interner_mut().intern("Wild Goose Chase");
    let bird_sub = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Draw two cards, then discard two cards. Create a Food token.".into(),
        target_requirements: vec![],
        modal: None,
        effect: wild_goose_chase_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent().tokens_only(),
                },
                intervening_if: None,
                effect: token_sacrificed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn token_sacrificed(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually gets +1/+1" (Arena-only mechanic) — not in catalog
    Vec::new()
}

fn wild_goose_chase_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
        Effect::CreateCommodityToken { controller: entry.controller, kind: CommodityToken::Food, count: 1 },
    ]
}
