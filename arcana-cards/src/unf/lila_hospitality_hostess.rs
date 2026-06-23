//! Lila, Hospitality Hostess — `{2}{G}{W}` 3/3 Legendary Creature — Elf
//! Employee. G/W.
//!
//! "You may look at the top card of your library any time." — a continuous
//! information static (GAP: no primitive).
//! "You may cast common spells from the top of your library." — a play-from-top
//! permission static (GAP: no primitive).
//! "Guests you control get +1/+1." — a static anthem restricted to the Guest
//! subtype. WIRED via an ETB-installed `ContinuousEffect::filtered_pump`
//! (filter: Guest creatures you control), lasting while Lila is on the
//! battlefield (CR 603 / glorious_anthem precedent).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lila, Hospitality Hostess");
    let elf = reg.interner_mut().intern("Elf");
    let employee = reg.interner_mut().intern("Employee");
    // Intern the Guest subtype for the anthem filter.
    let _guest = reg.interner_mut().intern("Guest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(employee);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "You may look at the top card of your library any time" —
    //      a continuous information static; no primitive on this shape.
    // GAP: static "You may cast common spells from the top of your library" —
    //      a play-from-top permission static; no primitive on this shape.
    reg.register(
        CardDefinition::new(name, chars)
            // "Guests you control get +1/+1." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_guest_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Guests you control get +1/+1" as a continuous effect anchored to
/// Lila, lasting while she remains on the battlefield.
fn install_guest_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let guest = reg.interner().lookup("Guest").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(guest);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
