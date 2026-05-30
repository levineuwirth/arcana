//! Invasion of Ixalan // Belligerent Regisaur
//!
//! Front face: Battle — Siege, {1}{G}, enters with 4 defense counters.
//! When this Siege enters, look at the top five cards of your library.
//! You may reveal a permanent card from among them and put it into your hand.
//! Put the rest on the bottom of your library in a random order.
//!
//! Back face: Creature — Dinosaur, Trample.
//! Whenever you cast a spell, this creature gains indestructible until end of turn.
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: Back-face-only triggered ability (gains indestructible when you cast a spell) not modeled.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Ixalan");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Belligerent Regisaur");
    let dinosaur_sub = reg.interner_mut().intern("Dinosaur");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dinosaur_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Trample],
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: Back-face-only triggered ability (gains Indestructible when you cast a spell) not modeled.
    )
}

fn etb_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Look at top 5, may reveal a permanent card and put it into hand, rest to bottom random.
    // "Permanent card" = creature, artifact, enchantment, land, battle (any of these types).
    let perm_filter = ObjectFilter::permanent().with_types_any(TypeLine(
        TypeLine::CREATURE
            | TypeLine::ARTIFACT
            | TypeLine::ENCHANTMENT
            | TypeLine::LAND
            | TypeLine::BATTLE,
    ));
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(perm_filter),
        rest: DigRest::BottomRandom,
    }]
}
