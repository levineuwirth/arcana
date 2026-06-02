//! Curse of the Werefox — `{2}{G}` sorcery. "Create a Monster Role
//! token attached to target creature you control. When you do, that
//! creature fights up to one target creature you don't control."
//!
//! The Role-token half (mint an Aura enchantment token attached to a
//! permanent, granting +1/+1 and trample, with the "another Role goes
//! to the graveyard" replacement) is not expressible: there is no
//! Effect to create a token ATTACHED to a target, no Aura/attachment
//! token primitive, and no Role bookkeeping. We DO express the
//! "fights up to one target creature you don't control" half, since
//! the two creatures are the spell's targets and `Effect::Fight` is a
//! catalog primitive. The fight is the reflexive ("when you do")
//! consequence; with both creatures available as targets it resolves
//! as a single fight.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curse of the Werefox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a Monster Role token attached to target creature you control. When you do, that creature fights up to one target creature you don't control.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
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
    // GAP: "Create a Monster Role token attached to target creature you
    // control" — no Effect mints an Aura/attachment (Role) token attached
    // to a target, nor models the +1/+1+trample grant or the replace-an-
    // existing-Role rule. We express the reflexive fight only.
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(mine)) = targets.next() else {
        return Vec::new();
    };
    // "up to one" — the opposing creature may be absent.
    let Some(TargetChoice::Object(theirs)) = targets.next() else {
        return Vec::new();
    };
    vec![Effect::Fight { a: *mine, b: *theirs }]
}
