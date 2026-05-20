//! Mythos of Nethroi — `{2}{B}` instant. "Destroy target nonland
//! permanent if it's a creature or if {G}{W} was spent to cast this
//! spell." The mana-spent condition is untrackable; the target is
//! constrained to a creature so the destroy is always legal, and the
//! {G}{W}-spent broadening of valid targets is gapped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mythos of Nethroi");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nonland permanent if it's a creature or if {G}{W} was spent to cast this spell.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: the "{G}{W} was spent" alternative (which would let any
    // nonland permanent be targeted) is untrackable; only the
    // creature case is implemented.
    vec![Effect::DestroyPermanent { target: *id }]
}
