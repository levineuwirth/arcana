//! Cartouche of Solidarity — `{W}` enchantment — Aura Cartouche.
//! "Enchant creature you control. When this Aura enters, create a 1/1 white
//!  Warrior creature token with vigilance. Enchanted creature gets +1/+1 and
//!  has first strike."
//!
//! Fully expressed: the fixed ETB trigger creates the Warrior token AND
//! installs the +1/+1 (attached_pt) and first strike (attached_keyword)
//! grants on the host. "Enchant creature you control" is approximated by the
//! caster's choice.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Cartouche of Solidarity");
    let aura = reg.interner_mut().intern("Aura");
    let cartouche = reg.interner_mut().intern("Cartouche");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    subtypes.0.insert(cartouche);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "enchant creature you control" approximated by caster's choice
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(warrior);
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: warrior,
                colors: ColorSet::white(),
                types: TypeLine(TypeLine::CREATURE),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Vigilance],
                abilities: Vec::new(),
            },
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::FirstStrike,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
