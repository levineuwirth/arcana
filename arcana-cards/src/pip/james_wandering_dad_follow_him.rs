//! James, Wandering Dad // Follow Him — `{2}{U}` // `{X}{U}{U}` blue Adventure creature.
//! Legendary Creature — Human Scientist. 2/4. "{T}: Add {C}{C}. Spend this mana only to activate abilities."
//! Adventure (Follow Him — Instant): Investigate X times.
//! GAP: "{T}: Add {C}{C} for abilities only" — restricted mana not in catalog; using plain mana.
//! GAP: "Investigate X times" — X determined from spell casting, not modeled.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("James, Wandering Dad");
    let adv_name = reg.interner_mut().intern("Follow Him");
    let human_sub = reg.interner_mut().intern("Human");
    let scientist_sub = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(scientist_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet(SupertypeSet::LEGENDARY), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(4)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{X}{U}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::INSTANT.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Investigate X times.".into(), target_requirements: vec![], modal: None, effect: follow_him_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_activated_ability(ActivatedAbilityDef { text: "{T}: Add {C}{C}.".into(), cost: ActivationCost { tap: true, ..ActivationCost::default() }, target_requirements: vec![], is_mana_ability: true, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: add_colorless })
            .with_adventure(adventure),
    )
}

fn add_colorless(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source), ManaUnit::plain(ManaColor::Colorless, ctx.source)] }]
}

fn follow_him_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    // GAP: X times — X value not accessible from StackEntry in this context
    vec![Effect::CreateCommodityToken { controller: entry.controller, kind: CommodityToken::Clue, count: 1 }]
}
