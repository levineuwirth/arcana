//! Brine Comber // Brinebound Gift — {1}{W}{U}
//!
//! Front: Creature — Spirit 1/1
//! Whenever this creature enters or becomes the target of an Aura spell,
//! create a 1/1 white Spirit creature token with flying.
//! Disturb {W}{U}
//!
//! Back: Enchantment — Aura
//! Enchant creature
//! Whenever this Aura enters or enchanted creature becomes the target of an Aura spell,
//! create a 1/1 white Spirit creature token with flying.
//! If this Aura would be put into a graveyard from anywhere, exile it instead.
//!
//! The "enters → create a 1/1 white Spirit with flying" token trigger fires on BOTH
//! faces (the front creature's ETB and the back Aura's ETB produce the identical token),
//! so the single ungated SelfEntersBattlefield trigger already covers the back-face Aura
//! ETB token — no separate face-gated trigger is needed.
//! GAP: "[creature] becomes the target of an Aura spell" — TriggerCondition::SelfBecomesTarget
//! only filters by the targeting player (caster), not by the targeting spell being an Aura;
//! firing on any targeting spell would over-fire, so this half is left unwired.
//! GAP: "If this Aura would be put into a graveyard from anywhere, exile it instead" — no
//! card-installable self-exile-instead-of-graveyard replacement (replacement::exile_instead
//! is a death-shield outcome, not a static an Aura can install on itself).
//! GAP: Disturb (cast from graveyard transformed) — no cast-from-graveyard-transformed mechanic.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brine Comber");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Brinebound Gift");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Enters trigger (fires on both faces) — create a 1/1 white Spirit with
            // flying. Covers the front creature's ETB and the back Aura's ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_spirit_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: "becomes the target of an Aura spell" — no Aura-spell-filtered
        // becomes-target trigger; left unwired to avoid over-firing.
    )
}

fn create_spirit_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit")
        .expect("Spirit interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
