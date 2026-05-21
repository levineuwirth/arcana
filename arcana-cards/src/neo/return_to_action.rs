//! Return to Action — `{1}{B}` instant. "Until end of turn, target
//! creature gets +1/+0 and gains lifelink and 'When this creature
//! dies, return it to the battlefield tapped under its owner's
//! control.'" We emit the +1/+0 + lifelink pump and the
//! "this-dies → return-from-graveyard" delayed action (tapped is
//! GAPped — DelayedAction returns untapped).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Return to Action");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, target creature gets +1/+0 and gains lifelink and \"When this creature dies, return it to the battlefield tapped under its owner's control.\"".into(),
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
    // GAP: the granted dies-trigger that returns *tapped* — DelayedAction returns from graveyard untapped; "tapped" rider lost.
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Lifelink],
        },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::ThisDies,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
