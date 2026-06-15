//! Squire's Devotion — `{2}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 and has lifelink.
//!  When this Aura enters, create a 1/1 white Vampire creature token with
//!  lifelink."
//!
//! Buff Aura. The ETB trigger installs the +1/+1 (attached_pt) and the
//! lifelink grant (attached_keyword), and creates the Vampire token.

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
    let name = reg.interner_mut().intern("Squire's Devotion");
    let aura = reg.interner_mut().intern("Aura");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                effect: etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(trig.source, 1, 1, Duration::WhileSourceOnBattlefield),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Lifelink,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
