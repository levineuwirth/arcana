//! Kellan, Inquisitive Prodigy // Tail the Suspect — `{2}{G}{U}` // `{G}{U}` green/blue Adventure creature.
//! Legendary Creature — Human Faerie Detective. 3/4. Flying, vigilance.
//! Whenever Kellan attacks, destroy up to one target artifact. If you controlled that permanent, draw a card.
//! Adventure (Tail the Suspect — Sorcery): Investigate. You may play an additional land this turn.
//! GAP: "if you controlled that permanent, draw a card" — ownership check not in catalog.
//! GAP: "you may play an additional land this turn" — additional-land-play not in catalog.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kellan, Inquisitive Prodigy");
    let adv_name = reg.interner_mut().intern("Tail the Suspect");
    let human_sub = reg.interner_mut().intern("Human");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let detective_sub = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(faerie_sub);
    subtypes.0.insert(detective_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Investigate. You may play an additional land this turn.".into(),
        target_requirements: vec![],
        modal: None,
        effect: tail_the_suspect_resolve,
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
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: kellan_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_adventure(adventure),
    )
}

fn kellan_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "if you controlled that permanent, draw a card" — ownership check not in catalog
    vec![Effect::DestroyPermanent { target: *id }]
}

fn tail_the_suspect_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may play an additional land this turn" — additional-land-play not in catalog
    vec![Effect::CreateCommodityToken {
        controller: entry.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
