//! Okiba Salvage — `{4}{B}` sorcery. "Return target creature or
//! Vehicle card from your graveyard to the battlefield. Then put two
//! +1/+1 counters on that permanent if you control an artifact and an
//! enchantment."
//!
//! The conditional +1/+1 counters (only if you control an artifact
//! and an enchantment, and applied to the just-reanimated permanent
//! whose new id isn't available) are not expressible; only the
//! reanimation is emitted.

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
    let name = reg.interner_mut().intern("Okiba Salvage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature or Vehicle card from your graveyard to the battlefield. Then put two +1/+1 counters on that permanent if you control an artifact and an enchantment.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: conditional +1/+1 counters on the reanimated permanent
    // (new battlefield id unavailable) not expressible.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
