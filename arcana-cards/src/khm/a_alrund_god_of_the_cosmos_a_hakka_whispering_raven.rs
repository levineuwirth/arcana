//! A-Alrund, God of the Cosmos // A-Hakka, Whispering Raven
//!
//! Front face: `{3}{U}{U}` blue Legendary Creature — God 1/1.
//! "Alrund gets +1/+1 for each card in your hand and each foretold card you own in exile."
//! "At the beginning of your end step, choose a card type, then reveal the top three cards
//!  of your library. Put all cards of the chosen type revealed this way into your hand and
//!  the rest on the bottom of your library in any order."
//!
//! Back face: A-Hakka, Whispering Raven — Legendary Creature — Bird with Flying.
//! "Whenever Hakka deals combat damage to a player, return it to its owner's hand, then scry 2."
//!
//! GAP: Alrund's +1/+1 per-card-in-hand static P/T bonus — continuous layer modifier not in engine.
//!      Static P/T stored as printed 1/1.
//! GAP: "each foretold card you own in exile" — Foretell mechanic not modeled.
//! GAP: Alrund's end-step "choose a card type" ability — card-type choice not expressible;
//!      entire ability omitted.
//! GAP: Hakka's triggered ability — back-face-only triggered ability not auto-installed
//!      (engine debt: triggered abilities live on CardDefinition, not per-face).
//!      Authored here on the CardDefinition; will fire on both faces.
//! GAP: Scry keyword (Scryfall-listed) not in implemented keyword set.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Alrund, God of the Cosmos");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: +1/+1 per card in hand — continuous layer bonus not supported; stored as 1/1
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: A-Hakka, Whispering Raven — Legendary Creature — Bird
    let back_name = reg.interner_mut().intern("A-Hakka, Whispering Raven");
    let bird_sub = reg.interner_mut().intern("Bird");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird_sub);

    let back_chars = Characteristics {
        name: back_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid back cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Alrund's end-step "choose a card type" ability — not expressible; omitted.
            // Hakka's triggered ability: "Whenever Hakka deals combat damage to a player,
            // return it to its owner's hand, then scry 2."
            // GAP: back-face-only trigger — authored on CardDefinition; fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: hakka_damage_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_mdfc_back(back_face)
    )
}

fn hakka_damage_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::ReturnToHand { target: trig.source },
        Effect::Scry { player: trig.controller, count: 2 },
    ]
}
