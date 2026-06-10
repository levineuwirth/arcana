//! Ride Down — `{R}{W}` instant. "Destroy target blocking creature.
//! Creatures that were blocked by that creature this combat gain
//! trample until end of turn." The attackers blocked by the target are
//! enumerated at resolution via `script::attackers_blocked_by` (current
//! combat pairing; "this combat" history for already-removed pairings is
//! approximated by the live pairing).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ride Down");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target blocking creature. Creatures that were blocked by that creature this combat gain trample until end of turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().blocking_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "Creatures that were blocked by that creature this combat" — the
    // attackers the target is blocking, captured before the destroy.
    let mut effects = vec![Effect::DestroyPermanent { target: *id }];
    effects.extend(
        script::attackers_blocked_by(state, *id)
            .into_iter()
            .map(|attacker| Effect::GrantKeyword {
                target: attacker,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            }),
    );
    effects
}
