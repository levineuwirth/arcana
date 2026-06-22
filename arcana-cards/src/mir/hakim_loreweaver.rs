//! Hakim, Loreweaver — `{3}{U}{U}` 2/4 Legendary Human Wizard.
//! Flying.
//! {U}{U}: Return target Aura card from your graveyard to the battlefield
//! attached to Hakim. Activate only during your upkeep and only if Hakim
//! isn't enchanted.
//! {U}{U}, {T}: Destroy all Auras attached to Hakim.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hakim, Loreweaver");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let aura_filter = script::subtype_filter(reg, "Aura");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: activation gates "only during your upkeep and only if
            // Hakim isn't enchanted" not expressible via the documented
            // conditions helpers; ability offered unconditionally.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{U}: Return target Aura card from your graveyard to the battlefield attached to Hakim.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: aura_filter,
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_aura_attached,
            })
            // GAP: "Destroy all Auras attached to Hakim" — no documented
            // filter for "Auras attached to this source"; effect omitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{U}, {T}: Destroy all Auras attached to Hakim.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_auras_on_self,
            }),
    )
}

fn return_aura_attached(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::Attach { equipment_or_aura: *id, target: ctx.source },
    ]
}

fn destroy_auras_on_self(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Destroy all Auras attached to Hakim" — no documented filter
    // selecting permanents attached to a given source.
    Vec::new()
}
