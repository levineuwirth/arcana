//! Squirming Emergence — `{1}{B}{G}` sorcery. "Fathomless descent —
//! Return to the battlefield target nonland permanent card in your
//! graveyard with mana value less than or equal to the number of
//! permanent cards in your graveyard."
//!
//! Reanimates a nonland permanent card from your graveyard to the
//! battlefield. The fathomless-descent mana-value gate (MV ≤ number of
//! permanent cards in your graveyard) is a dynamic per-target
//! restriction on the legal targets that the target-filter API can't
//! express (it allows fixed type/color/cmc filters, not a cmc bound
//! computed from graveyard contents), so the target is filtered only to
//! a nonland permanent card and the dynamic MV cap is left as a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squirming Emergence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Fathomless descent — Return to the battlefield target nonland permanent card in your graveyard with mana value less than or equal to the number of permanent cards in your graveyard.".into(),
                // GAP: cannot express "mana value <= number of permanent
                // cards in your graveyard" — target filter only supports
                // a fixed cmc bound, not one computed from graveyard size.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
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
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
