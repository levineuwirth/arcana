//! Spellscorn Coven // Take It Back — `{3}{B}` Faerie Warlock creature 2/3 with Flying.
//! When this creature enters, each opponent discards a card.
//! Adventure face "Take It Back" `{2}{U}` instant: Return target spell to its owner's hand.
//!
//! # Notes
//! The creature face is black; the Adventure face is blue ({2}{U}).
//! Colors on Adventure face = blue per the mana cost.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellscorn Coven");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie_sub);
    subtypes.0.insert(warlock_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Take It Back");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid adv cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Return target spell to its owner's hand.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Spell(ObjectFilter::new()),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: take_it_back_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|opp| Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect()
}

fn take_it_back_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}
