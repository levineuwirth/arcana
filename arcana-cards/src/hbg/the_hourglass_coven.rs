//! The Hourglass Coven — `{4}{B}{B}` 3/3 Legendary Hag Warlock.
//! When The Hourglass Coven enters the battlefield, draft a card from The
//! Hourglass Coven's spellbook twice, then put those cards onto the
//! battlefield.
//! Other Warlocks you control get +1/+1.
//!
//! * The ETB "draft a card from ~'s spellbook twice, then put onto the
//!   battlefield" is an Alchemy spellbook mechanic with no expressible
//!   primitive (no registry-by-spellbook lookup) — GAP'd.
//! * "Other Warlocks you control get +1/+1" is a static anthem, wired as a
//!   `SelfEntersBattlefield` trigger that installs a
//!   `ContinuousEffect::filtered_pump` over Warlocks you control. (The
//!   "other" exclusion — excluding this Warlock itself — is a documented
//!   minor fidelity gap; the filter matches base characteristics.)

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
    let name = reg.interner_mut().intern("The Hourglass Coven");
    let hag = reg.interner_mut().intern("Hag");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: ETB "draft a card from ~'s spellbook twice, then put onto the
    // battlefield" — Alchemy spellbook draft, no expressible primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_warlock_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Warlocks you control get +1/+1" anchored to this creature,
/// lasting until it leaves the battlefield.
fn install_warlock_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warlock = reg.interner().lookup("Warlock").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(warlock);
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
