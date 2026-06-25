//! Craving of Yeenoghu — `{2}{R}` enchantment — Aura.
//! "Enchant creature you control. Enchanted creature gets +3/+2, has haste,
//!  and attacks each combat if able. {R}: Return Craving of Yeenoghu from your
//!  graveyard to the battlefield attached to target creature you control.
//!  Craving of Yeenoghu perpetually gains \"Enchanted creature gets -1/-1.\"
//!  Activate only as a sorcery."
//!
//! ETB installs `attached_pt(+3, +2)`, `attached_keyword(Haste)`, and a
//! must-attack continuous effect on the enchanted creature (host captured at
//! ETB). The graveyard-return-with-perpetual activated ability has no
//! expressible primitive here — GAP'd.

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
    let name = reg.interner_mut().intern("Craving of Yeenoghu");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "creature you control" approximated by caster's Creature choice.
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
    // GAP: "{R}: Return from graveyard to battlefield attached … perpetually
    //      gains -1/-1; sorcery speed" — no graveyard-return/perpetual primitive.
    let mut effects = vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                3,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ];
    // "and attacks each combat if able" — must-attack on the enchanted
    // creature for as long as this Aura stays attached (host captured at ETB).
    if let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(
                trig.source,
                host,
                Duration::WhileSourceOnBattlefield,
            ),
        });
    }
    effects
}
