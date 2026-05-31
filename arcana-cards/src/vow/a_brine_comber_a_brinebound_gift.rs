//! A-Brine Comber // A-Brinebound Gift (transforming DFC, Disturb).
//! Front (A-Brine Comber — Creature — Spirit, 2/2, {1}{W}{U}):
//!   Whenever this creature enters or becomes the target of an Aura spell,
//!   create a 1/1 white Spirit creature token with flying.
//!   Disturb {W}{U}.
//! Back (A-Brinebound Gift — Enchantment — Aura, Enchant creature):
//!   Whenever this Aura enters or enchanted creature becomes the target of an
//!   Aura spell, create a 1/1 white Spirit creature token with flying.
//!   If Brinebound Gift would be put into a graveyard from anywhere, exile it instead.
//!
//! GAPs:
//! - Disturb keyword (alternative cast from graveyard transformed): not in the engine's
//!   keyword surface; not modeled.
//! - "becomes the target of an Aura spell": modeled via SelfBecomesTarget, but the engine
//!   condition is not restricted to Aura spells (any spell/ability targeting it fires it).
//! - "If Brinebound Gift would be put into a graveyard, exile it instead": replacement
//!   effect; not expressible.
//! - Back face is an Aura (Enchant creature). The back's own enters / enchanted-creature
//!   targeted trigger is not auto-installed on transform and Aura attachment of the
//!   transformed back face is not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Brine Comber");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    // Pre-intern token subtype for resolution.
    let _ = reg.interner_mut().intern("Spirit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: A-Brinebound Gift — Enchantment — Aura.
    let back_name = reg.interner_mut().intern("A-Brinebound Gift");
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
            // GAP: Enchant creature attachment of transformed back-face Aura not modeled.
            // GAP: "If Brinebound Gift would be put into a graveyard, exile it instead"
            //   — replacement effect not expressible.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: "Whenever this creature enters, create a 1/1 white Spirit with flying."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front: "Whenever this creature becomes the target of an Aura spell, ..."
            // GAP: not restricted to Aura spells (any spell/ability targeting fires it).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0),
    )
}

fn make_spirit(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit_sym = reg.interner().lookup("Spirit").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spirit_sym);
    let token = TokenDefinition {
        name: spirit_sym,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
