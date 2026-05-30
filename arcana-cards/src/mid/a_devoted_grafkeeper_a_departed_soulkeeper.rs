//! A-Devoted Grafkeeper // A-Departed Soulkeeper — `{W}{U}` Human Peasant 2/2.
//! Front:
//!   When Devoted Grafkeeper enters, mill four cards.
//!   "Whenever you cast a spell from your graveyard, tap target creature you don't control."
//!     GAP: TriggerCondition::SpellCast has no from_zone filter; can't restrict to graveyard cast.
//!   Disturb {1}{W}{U} — GAP: Disturb keyword not in the supported keyword surface.
//! Back (A-Departed Soulkeeper): Flying Spirit.
//!   "If Departed Soulkeeper would be put into a graveyard from anywhere, exile it instead."
//!     GAP: replacement effect (graveyard -> exile redirect) not expressible via Effect catalog.
//!   GAP: back-face-only abilities not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Devoted Grafkeeper");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Disturb {1}{W}{U} keyword not in supported keyword surface
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("A-Departed Soulkeeper");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: mill four cards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            // GAP: "Whenever you cast a spell from your graveyard, tap target creature
            // you don't control." — SpellCast trigger has no from_zone filter for graveyard.
    )
}

fn etb_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 4,
    }]
}
