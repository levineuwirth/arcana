//! Utter Insignificance — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant creature. Enchanted creature loses all abilities and has
//!  base power and toughness 1/1. {2}{C}: Exile enchanted creature."
//!
//! Flash is a card keyword. The "base power and toughness 1/1" half is an
//! ETB-installed `attached_set_pt(1, 1)` that follows `source.attached_to`.
//! "Loses all abilities" has no attached-to-host builder (only a targeted
//! `lose_all_abilities`, which doesn't follow the attachment) and the
//! "{2}{C}: exile enchanted creature" is the Aura's own activated ability —
//! both stay GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Utter Insignificance");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses all abilities" — no attached-to-host lose-all-abilities
    // builder; "{2}{C}: exile enchanted creature" — the Aura's own activated
    // ability. The base-P/T-setting half is wired below.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_set_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
