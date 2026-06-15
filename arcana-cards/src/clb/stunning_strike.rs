//! Stunning Strike — `{2}{U}` enchantment — Aura.
//! "Flash. Enchant creature. When this Aura enters, tap enchanted
//! creature and remove it from combat. As long as enchanted creature
//! isn't legendary, it doesn't untap during its controller's untap
//! step."
//!
//! Flash is on the card. The ETB taps the enchanted host (read via
//! `source.attached_to`). "Remove from combat" and the conditional
//! doesn't-untap lock have no demonstrated builder, so both are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Stunning Strike");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "remove it from combat" and the conditional doesn't-untap lock
    // ("as long as ... isn't legendary") have no demonstrated builder.
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    vec![Effect::Tap { target: host }]
}
