//! Besotted Knight // Betroth the Beast — `{3}{W}` // `{W}` white Adventure creature.
//! Creature: 3/3 Human Knight. No other abilities.
//! Adventure (Betroth the Beast — Sorcery): Create a Royal Role token attached to target creature you control.
//! GAP: Royal Role token — Role Enchantment Aura token type not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Besotted Knight");
    let adv_name = reg.interner_mut().intern("Betroth the Beast");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a Royal Role token attached to target creature you control.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: betroth_the_beast_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure),
    )
}

fn betroth_the_beast_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Royal Role token — Role Enchantment Aura token type not in catalog
    Vec::new()
}
