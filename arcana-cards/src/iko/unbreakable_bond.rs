//! Unbreakable Bond — `{4}{B}` sorcery. "Return target creature card
//! from your graveyard to the battlefield with a lifelink counter on
//! it." The lifelink counter is wired as `CounterKind::Named("lifelink")`
//! placed as the card re-enters (`ReturnFromGraveyardWithCounters`).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unbreakable Bond");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("lifelink");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to the battlefield with a lifelink counter on it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(kind) = reg.interner().lookup("lifelink").map(CounterKind::Named) else {
        return vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }];
    };
    // GAP: the lifelink KEYWORD grant from the keyword counter (CR 122.1g)
    // is not wired — the zone move re-ids the object, so a follow-up
    // GrantKeyword can't reach the fresh battlefield id.
    vec![Effect::ReturnFromGraveyardWithCounters {
        target: *id,
        kind,
        count: 1,
    }]
}
