//! Firewake Sliver — `{1}{R}{G}` 1/1 Sliver.
//!
//! * All Sliver creatures have haste.
//!   Wired via a SelfEntersBattlefield trigger installing a
//!   `ContinuousEffect::filtered_keyword` granting Haste to all Sliver
//!   creatures (no controller constraint — "All Slivers", any controller),
//!   lasting while this creature is on the battlefield.
//! * All Slivers have "{1}, Sacrifice this permanent: Target Sliver creature
//!   gets +2/+2 until end of turn."
//!   // GAP: granting a static ACTIVATED ability to a group of permanents has
//!   no expressible form (only keyword grants exist as filtered continuous
//!   effects); unmodeled.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Firewake Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "All Slivers have '{1}, Sacrifice this permanent: Target Sliver
    // creature gets +2/+2 until end of turn.'" — granting a static activated
    // ability to a group of permanents has no expressible form.

    reg.register(
        CardDefinition::new(name, chars)
            // "All Sliver creatures have haste."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_sliver_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_sliver_haste(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let sliver = reg.interner().lookup("Sliver").expect("Sliver interned at register");
    let filter = ObjectFilter::creature().with_subtype_sym(sliver);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            filter,
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
