//! Conceited Witch // Price of Beauty — `{2}{B}` // `{B}` black Adventure creature.
//! Creature: 2/3 Human Warlock. Menace.
//! Adventure (Price of Beauty — Sorcery): Create a Wicked Role token attached to target creature you control.
//! GAP: Wicked Role token — Role token type not in catalog; using placeholder token.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conceited Witch");
    let adv_name = reg.interner_mut().intern("Price of Beauty");
    let human_sub = reg.interner_mut().intern("Human");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let _wicked = reg.interner_mut().intern("Wicked");
    let _role = reg.interner_mut().intern("Role");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warlock_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a Wicked Role token attached to target creature you control.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: price_of_beauty_resolve,
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

fn price_of_beauty_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Wicked Role token — Role Enchantment Aura token type not in catalog
    Vec::new()
}
