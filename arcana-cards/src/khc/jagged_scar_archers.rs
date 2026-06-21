//! Jagged-Scar Archers — `{1}{G}{G}` */* green Elf Archer.
//! "Jagged-Scar Archers's power and toughness are each equal to the number
//! of Elves you control. {T}: This creature deals damage equal to its power
//! to target creature with flying."
//!
//! The */* characteristic-defining ability (CR 604.3) that sets P/T to the
//! number of Elves you control is a static and is not expressible with the
//! demonstrated primitives — recorded as a GAP. The tap-to-ding-a-flyer
//! activated ability deals damage equal to the creature's current power via
//! `script::power_of`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jagged-Scar Archers");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);

    // GAP: static CDA — power and toughness each equal to the number of
    // Elves you control. Modeled as */* bones; the count is unexpressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: This creature deals damage equal to its power to \
                   target creature with flying."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ding_flyer,
        }),
    )
}

fn ding_flyer(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let amount = script::power_of(state, ctx.source).max(0) as u32;
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
