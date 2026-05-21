//! Gruesome Encore — `{2}{B}` sorcery. "Put target creature card from
//! an opponent's graveyard onto the battlefield under your control.
//! It gains haste. Exile it at the beginning of the next end step. If
//! that creature would leave the battlefield, exile it instead of
//! putting it anywhere else." Reanimate-from-opponent's-graveyard +
//! end-of-turn-exile + leave-the-battlefield-replacement: the catalog
//! has ReturnFromGraveyardToBattlefield + DelayedAction::Exile so we
//! can express the first two beats; haste grant is doable too. The
//! 'leaves the battlefield, exile instead' replacement is not in the
//! catalog — emit the bones and GAP the replacement.

use arcana_core::effects::{
    DelayedAction, DelayedWhen, Effect, KeywordAbility,
};
use arcana_core::layers::Duration;
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
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gruesome Encore");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put target creature card from an opponent's graveyard onto the battlefield under your control. It gains haste. Exile it at the beginning of the next end step. If that creature would leave the battlefield, exile it instead of putting it anywhere else.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: 'if it would leave the battlefield, exile it instead'
    // replacement effect on the reanimated creature.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ]
}
