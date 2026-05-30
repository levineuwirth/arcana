//! Stormkeld Curator // Giant Secrets
//!
//! Creature face: `{4}{W}{W}` Creature — Giant 6/6.
//! When Stormkeld Curator enters, you may put any number of Aura cards from
//! your graveyard and/or hand onto the battlefield attached to it.
//! GAP: "put Aura cards from graveyard/hand onto the battlefield attached to it"
//!      — no bulk-Aura-reanimate/play effect in the catalog; not expressible.
//!
//! Adventure face: "Giant Secrets" `{X}{U}{U}` Instant.
//! Conjure X random cards from Giant Secrets's spellbook into your hand.
//! GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name
//!      lookup in Effect::execute).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormkeld Curator");
    let giant_sub = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // Adventure face: Giant Secrets — {X}{U}{U} Instant.
    // GAP: Conjure not modeled.
    let adv_name = reg.interner_mut().intern("Giant Secrets");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{X}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Conjure X random cards from Giant Secrets's spellbook into your hand.".into(),
        target_requirements: vec![],
        modal: None,
        effect: giant_secrets_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn giant_secrets_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name
    // lookup in Effect::execute).
    Vec::new()
}
