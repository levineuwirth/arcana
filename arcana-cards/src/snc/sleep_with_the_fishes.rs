//! Sleep with the Fishes — `{2}{U}{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, tap enchanted creature and
//!  you create a 1/1 blue Fish creature token with 'This token can't be
//!  blocked.' Enchanted creature doesn't untap during its controller's
//!  untap step."
//!
//! The ETB trigger taps the host (`source.attached_to`), creates the Fish
//! token, and installs `attached_dont_untap`. The token's "can't be
//! blocked" granted ability is not expressible on TokenDefinition — GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sleep with the Fishes");
    let aura = reg.interner_mut().intern("Aura");
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Fish token's "can't be blocked" granted ability not expressible
    // on TokenDefinition.
    let fish = reg.interner().lookup("Fish").unwrap_or_default();
    let mut tok_sub = SubtypeSet::default();
    tok_sub.0.insert(fish);
    let mut effects = Vec::new();
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::Tap { target: host });
    }
    effects.push(Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: fish,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: tok_sub,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    });
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
