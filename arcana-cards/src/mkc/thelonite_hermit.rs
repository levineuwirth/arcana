//! Thelonite Hermit — `{3}{G}` 1/1 green Elf Shaman.
//!
//! Oracle:
//! * All Saprolings get +1/+1. (static anthem — NOW WIRED via an ETB-installed
//!   `ContinuousEffect::filtered_pump` over all Saprolings.)
//! * Morph {3}{G}{G} (Morph keyword not in the supported surface — GAP)
//! * When this creature is turned face up, create four 1/1 green Saproling
//!   creature tokens. (no "turned face up" trigger condition — GAP)
//!
//! "All Saprolings" is board-wide (not controller-scoped), so the pump filter
//! is `creature().with_subtype_sym(saproling)` with NO `controlled_by`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thelonite Hermit");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    // Intern "Saproling" now so the effect fn's lookup is guaranteed to hit.
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Morph {3}{G}{G} — Morph not in the supported keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "When turned face up, create four Saproling tokens" — no
    //      turned-face-up TriggerCondition variant exists.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_saproling_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "All Saprolings get +1/+1" (board-wide, not
/// controller-scoped), anchored to this creature.
fn etb_install_saproling_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling = reg
        .interner()
        .lookup("Saproling")
        .expect("Saproling interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().with_subtype_sym(saproling),
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
